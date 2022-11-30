fn main()  -> Result<(),Box<dyn std::error::Error>>{
    for i in 0..18{
        let x = i % 9;
        println!("i {} -> {}",i,x);
    }


    Ok(())
}