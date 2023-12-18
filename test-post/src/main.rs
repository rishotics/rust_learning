mod app;

use app::{TestApp, start_rpc_server};

use tokio::runtime::Runtime;


fn main() {
    

    let rt  = Runtime::new().unwrap();
    let mut app = TestApp::new();
    let app_clone = app.clone();

    rt.block_on(async move {
        tokio::spawn(async move {
            app.start().await;
        });

        start_rpc_server(app_clone).await;
    });
   
}
