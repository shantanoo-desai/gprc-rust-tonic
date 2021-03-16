use ecommerce::product_info_client::{ProductInfoClient};
use ecommerce::{Product, ProductId};

mod ecommerce;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    println!("Connecting to gRPC Server");
    let mut client = ProductInfoClient::connect("http://[::1]:50051").await?;

    let request_add_product = tonic::Request::new(Product{
        id: "200".into(),
        name: "Product 200".into(),
        description: "Amazing Product: 200".into()
    });

    println!("Adding New Product...");

    let response_new_product = client.add_product(request_add_product).await?.into_inner();

    let uuid_to_search = response_new_product.clone();

    println!("Response for New Added Product:\n {:#?}", response_new_product);

    let request_find_product = tonic::Request::new(ProductId { value: uuid_to_search.value });
    
    let response_found_product = client.get_product(request_find_product).await?.into_inner();

    println!("Response for Found Product:\n {:#?}", response_found_product);

    Ok(())

    
}