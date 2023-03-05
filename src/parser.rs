use nom::{bytes::complete::*, character, IResult};

#[derive(Debug)]
pub struct IpAddress {
    pub octets: Vec<u8>,
    pub text: String,
}

pub fn parse_ip_address(input: &str) -> IResult<&str, IpAddress> {
    let (input, octets) = octet_line(input)?;
    let list = octets
        .clone()
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>();

    Ok((
        input,
        IpAddress {
            octets,
            text: list.join("."),
        },
    ))
}

fn octet_line(input: &str) -> IResult<&str, Vec<u8>> {
    let (input, oct1) = octet(input)?;
    let (input, _) = tag(".")(input)?;

    let (input, oct2) = octet(input)?;
    let (input, _) = tag(".")(input)?;

    let (input, oct3) = octet(input)?;
    let (input, _) = tag(".")(input)?;

    let (input, oct4) = octet(input)?;

    let v = vec![oct1, oct2, oct3, oct4];
    Ok((input, v))
}

fn octet(input: &str) -> IResult<&str, u8> {
    Ok(character::complete::u8(input)?)
}
