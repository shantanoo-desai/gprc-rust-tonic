use tonic::{transport::Server, Request, Response, Status};

use uuid::Uuid;

use std::sync::{Arc, Mutex};
use std::collections::HashMap;

/**
 * Generated gRPC Stub
 */

// Stub Import Syntax: <package_name>::<service_name_server>::{ServiceName, ServiceNameServer};
use ecommerce::product_info_server::{ProductInfo, ProductInfoServer};

// Stub Import Syntax: <package_name>::<Exact_Name_of_Message_In_Proto_File>
use ecommerce::{Product, ProductId};

mod ecommerce;


/**
 * In-Memory Temporary DB
 */

#[derive(Clone, Debug, Default)]
pub struct InternalProduct {
    name: String,
    description: String,
}

 #[derive(Clone, Debug, Default)]
 pub struct Db {
     shared: Arc<Shared>,
 }

 #[derive(Debug, Default)]
 struct Shared {
     products: Mutex<HashMap<String, InternalProduct>>
 }


 impl Db {
     pub fn new() -> Self {
         Self {
             shared: Arc::new(Shared {
                 products: Mutex::new(HashMap::new())
             })
         }
     }

     pub fn insert(&self, key: String, prod: InternalProduct) {
         let mut products = self.shared.products.lock().unwrap();
         products.insert(key, prod);
     }

     pub fn get(&self, key: String) -> Option<(String, InternalProduct)> {
         let products = self.shared.products.lock().unwrap();
         match products.get(&key) {
             Some(product) => Some((key, product.clone())),
             None => None,
         }
     }
 }

// Implement Service Skeleton for the "ProductInfo" Service
// Defined in the Proto File
#[derive(Debug, Default)]
pub struct MyProductInfo {
    product_map: Db,
}

// Implement the service functions defined in the Proto File

// addProduct, getProduct Service

#[tonic::async_trait]
impl ProductInfo for MyProductInfo {
    async fn add_product(&self, request: Request<Product>) -> Result<Response<ProductId>, Status> {
        println!("Received Request (addProduct): {:?}", request.remote_addr());

        // generate a new UUID v4
        let new_id = Uuid::new_v4();

        // Extract the Incoming User Data of the Product
        let mut new_product = request.into_inner();
        // Replace the ID of the Product with generated UUID
        new_product.id = new_id.clone().to_string();
        let internal_product = InternalProduct { name: new_product.name, description: new_product.description };
        self.product_map.insert(new_product.id, internal_product);
        let response = ecommerce::ProductId { value: new_id.clone().to_string()};
        Ok(Response::new(response))
    }

    async fn get_product(&self, request: Request<ProductId>) -> Result<Response<Product>, Status> {
        println!("Received Request (getProduct): {:?}", request.remote_addr());

        let search_result = self.product_map.get(request.into_inner().value);
        match search_result {
            Some((id, product)) => {
                Ok(
                    Response::new(
                        ecommerce::Product {
                            id: id,
                            name: product.name,
                            description: product.description,
                        }
                    )
                )
            },
            None => { Err(Status::not_found("Product Not Found")) }
        }

    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let ecomm_service = MyProductInfo::default();

    println!("Starting gRPC Server....");
    Server::builder()
            .add_service(ProductInfoServer::new(ecomm_service))
            .serve(addr)
            .await?;

    Ok(())
}
