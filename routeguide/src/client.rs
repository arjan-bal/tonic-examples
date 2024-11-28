use rand::{rngs::ThreadRng, Rng};
use routeguide::{route_guide_client::RouteGuideClient, Point, Rectangle, RouteNote};
use tokio::time;
use tonic::{transport::Channel, Request};

pub mod routeguide {
    tonic::include_proto!("routeguide");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = RouteGuideClient::connect("http://127.0.0.1:10000").await?;

    let response = client
        .get_feature(Request::new(Point {
            latitude: 409146138,
            longitude: -746188906,
        }))
        .await?;

    println!("Response = {:?}", response);

    print_features(&mut client).await?;
    run_record_route(&mut client).await?;
    run_route_chat(&mut client).await?;
    Ok(())
}

async fn print_features(
    client: &mut RouteGuideClient<Channel>,
) -> Result<(), Box<dyn std::error::Error>> {
    let rect = Rectangle {
        lo: Some(Point {
            latitude: 400000000,
            longitude: -750000000,
        }),
        hi: Some(Point {
            latitude: 420000000,
            longitude: -730000000,
        }),
    };

    let mut stream = client.list_features(Request::new(rect)).await?.into_inner();

    while let Some(feature) = stream.message().await.unwrap() {
        println!("FEATURE: {:?}", feature);
    }

    Ok(())
}

async fn run_record_route(
    client: &mut RouteGuideClient<Channel>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::thread_rng();
    let point_count = rng.gen_range(2..100);

    let points: Vec<_> = (0..point_count).map(|_| random_point(&mut rng)).collect();

    println!("Traversing {} points", points.len());
    let summary = client
        .record_route(Request::new(tokio_stream::iter(points)))
        .await?
        .into_inner();

    println!("SUMMARY = {:?}", summary);
    Ok(())
}

fn random_point(rng: &mut ThreadRng) -> Point {
    Point {
        latitude: (rng.gen_range(0..180) - 90) * 10_000_000,
        longitude: (rng.gen_range(0..360) - 180) * 10_000_000,
    }
}

async fn run_route_chat(
    client: &mut RouteGuideClient<Channel>,
) -> Result<(), Box<dyn std::error::Error>> {
    let start = time::Instant::now();
    let outbound = async_stream::stream! {
        let mut interval = time::interval(time::Duration::from_secs(1));

        while let time = interval.tick().await {
            let elapsed = time.duration_since(start);
            let note = RouteNote {
                location: Some(Point {
                    latitude: 409146138 + elapsed.as_secs() as i32,
                    longitude: -746188906,
                }),
                message: format!("at {:?}", elapsed),
            };
            yield note;
        }

    };

    let mut inbound = client
        .route_chat(Request::new(outbound))
        .await?
        .into_inner();

    while let Some(note) = inbound.message().await? {
        println!("NOTE = {:?}", note);
    }

    Ok(())
}
