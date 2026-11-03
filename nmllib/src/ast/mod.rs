
#[derive(Debug)]
pub enum NmlBlock {
    Grf(GrfMetadata),
}



#[derive(Debug)]
pub enum Unit {
    Nfo,
    Mph,
    Kmh,
    Ms,
    Hpi,
    Hpm,
    Hp,
    Kw,
    Tons,
    Ton,
    Kg,
    SnowPct,
}

#[derive(Debug)]
pub enum Translation {
    Literal(String),
    Id(String),
}

#[derive(Debug)]
pub enum Expression {
    Translation(Translation),
    Number(i64)
}

#[derive(Debug)]
pub struct NmlModule {
    blocks: Vec<NmlBlock>
}

#[derive(Debug)]
pub struct GrfMetadata {

}

