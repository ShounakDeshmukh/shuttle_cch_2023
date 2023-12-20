
// task 11 extra

use axum::extract::Multipart;

pub async fn bull_mode(mut multipart:Multipart){
    
    while let Some(mut field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        println!("Length of `{}` is {} bytes", name, data.len());

        println!("{:?}",data)
    }
}

