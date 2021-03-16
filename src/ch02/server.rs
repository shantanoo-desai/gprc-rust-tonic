use tonic::{transport::Server, Request, Response, Status};

/**
 * Generated gRPC Stub / Skeleton
 */

// Stub Import Syntax: <package_name>::<service_name_server>::{ServiceName, ServiceNameServer};
use ordermgmt::order_management_server::{OrderManagement, OrderManagementServer};

// Import Message: <message_name>
use ordermgmt::{Order};

mod ordermgmt;

#[derive(Debug, Default)]
pub struct MyOrderManagementService {}

#[tonic::async_trait]
impl OrderManagement for MyOrderManagementService {
    async fn get_order(&self, _request: Request<String>) -> Result<Response<Order>, Status> {
        unimplemented!()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let ecomm_service = MyOrderManagementService::default();

    println!("Starting gRPC Server....");
    Server::builder()
            .add_service(OrderManagementServer::new(ecomm_service))
            .serve(addr)
            .await?;

    Ok(())
}