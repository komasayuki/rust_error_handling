
use anyhow::bail;

async fn sample(num:u32) -> anyhow::Result<u32>{

    if num > 10{
        Ok(num)
    }
    else if num < 10{
        Err(anyhow::anyhow!("to small error"))
    }
    else{
        bail!("I don't like {}!", num);
    }

}


#[derive(Debug, Clone)]
struct MyError;

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "my error")
    }
}

impl std::error::Error for MyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

fn make_my_error()  -> Result<u32, MyError>{
    return Err(MyError{})
}


fn handle_error()-> anyhow::Result<u32>{

    let num = make_my_error()?;
    Ok(num)

}





use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyThisError {
    #[error("data store disconnected")]
    Disconnect(#[from] std::io::Error),
    #[error("the data for key `{0}` is not available")]
    Redaction(String),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader {
        expected: String,
        found: String,
    },
    #[error("unknown data store error")]
    Unknown,
}



fn make_my_this_error()  -> anyhow::Result<u32>{
    Err(anyhow::anyhow!(MyThisError::Redaction("Hello".to_owned())))
}


fn handle_this_error()-> anyhow::Result<u32>{

    let num = make_my_this_error()?;
    Ok(num)

}



#[tokio::main]
async fn main() -> anyhow::Result<()>{

    let r1 = sample(20).await;
    println!("r1 {:?}", r1);

    let r2 = sample(10).await;
    println!("r2 {:?}", r2);

    let r3 = sample(0).await;
    println!("r3 {:?}", r3);


    let m = handle_error();
    println!("handle error {:?}", m);

    match m{
        Ok(num) => println!("num {}",num),
        Err(e) => {
            if let Some(e) = e.downcast_ref::<std::io::Error>() {
                eprintln!("std io error {:?}",e);
            }
            else if let Some(_) = e.downcast_ref::<MyError>(){
                eprintln!("my error {:?}",e);
            }
            else{
                eprintln!("anyhow error {:?}",e);
            }

        }
    }


    let m2 = handle_this_error();

    match m2{
        Ok(num) => println!("num {}",num),
        Err(e) => {
            if let Some(e) = e.downcast_ref::<MyThisError>() {
                eprintln!("my this error {:?}",e);
            }
            else{
                eprintln!("anyhow error {:?}",e);
            }

        }
    }



    Ok(())
}