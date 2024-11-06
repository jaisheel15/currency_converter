use std::io::{self,Write};
use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize,Debug)]
struct ApiResponse{
    conversion_rates:std::collections::HashMap<String,f64>,
}


async fn get_exchange_rate(api_key:&str,base_currency:&str)->Result<ApiResponse,Error>{
    let url = format!(" https://v6.exchangerate-api.com/v6/{}/latest/{}",api_key,base_currency);
    let response = reqwest::get(&url).await?;
    let api_response:ApiResponse = response.json().await?;
    Ok(api_response)
}

#[tokio::main]
async fn main(){
    let api_key = "7f9b16f2a31411c0ed826290";
    let mut input = String::new();

    print!("Enter the base currency: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    let base_currency = input.trim().to_string().to_uppercase();

    input.clear();
    print!("Enter the target currency: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    let target_currency = input.trim().to_string().to_uppercase()
    ;

    input.clear();
    print!("Enter the amount to convert: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let amount :f64 = input.trim().parse().unwrap();

    match get_exchange_rate(api_key, &base_currency).await {
        Ok(response)=>{
            if let Some(rate) = response.conversion_rates.get(&target_currency){
                let cconverted_amount = amount * rate;
                println!("{:.2} {} = {:.2} {}",amount,base_currency,cconverted_amount,target_currency);
            
        }
        else{
            println!("Failed to get exchange rate");
        }
    }
        Err(e)=>{
            println!("Error: {}",e);
        }
    }

}



