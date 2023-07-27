use base64::Engine;
use miette::IntoDiagnostic;

const HACKATTIC_ACCESS_TOKEN: &str = "9e33cf3a4bf6b619";

#[derive(Debug, Clone, serde::Deserialize)]
struct Challenge {
    bytes: String,
}

#[derive(Debug, Clone, serde::Serialize)]
struct Answer {
    int: i32,
    uint: u32,
    short: i16,
    float: f64,
    double: f64,
    big_endian_double: f64,
}

#[tokio::main]
async fn main() -> miette::Result<()> {
    let body: Challenge = reqwest::get(format!(
        "https://hackattic.com/challenges/help_me_unpack/problem?access_token={}",
        HACKATTIC_ACCESS_TOKEN
    ))
    .await
    .into_diagnostic()?
    .json()
    .await
    .into_diagnostic()?;

    dbg!(&body.bytes);

    let mut buffer = vec![];
    base64::engine::general_purpose::STANDARD
        .decode_vec(body.bytes, &mut buffer)
        .into_diagnostic()?;

    // dbg!(buffer);
    dbg!(&buffer, buffer.len());
    let the_int = buffer.drain(0..4).collect::<Vec<u8>>();
    let the_int: i32 = i32::from_le_bytes(
        the_int
            .try_into()
            .map_err(|_| miette::miette!("Failed to convert vec to array"))?,
    );

    let the_unsigned_int = buffer.drain(0..4).collect::<Vec<u8>>();
    let the_unsigned_int: u32 = u32::from_le_bytes(
        the_unsigned_int
            .try_into()
            .map_err(|_| miette::miette!("Failed to convert vec to array"))?,
    );

    let the_short = buffer.drain(0..2).collect::<Vec<u8>>();
    let the_short = i16::from_le_bytes(
        the_short
            .try_into()
            .map_err(|_| miette::miette!("Failed to convert vec to array"))?,
    );

    // Why does this line help??
    let randos: Vec<_> = buffer.drain(0..2).collect();
    assert_eq!(randos, vec![0, 0]);

    let the_float = buffer.drain(0..4).collect::<Vec<u8>>();
    println!("{:x?}", the_float);
    let the_float_array: [u8; 4] = the_float
        .try_into()
        .map_err(|_| miette::miette!("Failed to convert vec to array"))?;
    let the_float = f32::from_le_bytes(the_float_array);
    let the_float_64 = the_float as f64;
    println!("{:.}", the_float);
    println!("{:.}", the_float_64);

    dbg!(the_float, the_float_64);

    let the_double = buffer.drain(0..8).collect::<Vec<u8>>();
    let the_double: f64 = f64::from_le_bytes(
        the_double
            .try_into()
            .map_err(|_| miette::miette!("Failed to convert vec to array"))?,
    );

    let the_big_endian_double = buffer.drain(0..8).collect::<Vec<u8>>();
    let the_big_endian_double: f64 = f64::from_be_bytes(
        the_big_endian_double
            .try_into()
            .map_err(|_| miette::miette!("Failed to convert vec to array"))?,
    );

    let answer = Answer {
        int: the_int,
        uint: the_unsigned_int,
        short: the_short,
        float: the_float_64,
        double: the_double,
        big_endian_double: the_big_endian_double,
    };

    dbg!(the_float);
    dbg!(&answer);

    let client = reqwest::Client::new();
    let resp: String = client
        .post(format!(
            "https://hackattic.com/challenges/help_me_unpack/solve?access_token={}",
            HACKATTIC_ACCESS_TOKEN
        ))
        .json(&answer)
        .send()
        .await
        .into_diagnostic()?
        .text()
        .await
        .into_diagnostic()?;

    dbg!(resp);

    Ok(())
}
