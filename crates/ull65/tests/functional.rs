mod fixture;

#[cfg(test)]
mod tests {
    use super::fixture;

    use ull65::instruction::mos6502::Mos6502;
    use ull65::instruction::wdc65c02s::Wdc65c02s;

    #[test]
    fn test_functional_roms_mos6502() {
        fixture::run_fixtures_with::<Mos6502>(fixture::MOS_FIXTURES);
    }

    #[test]
    fn test_functional_roms_wdc65c02() {
        fixture::run_fixtures_with::<Wdc65c02s>(fixture::WDC65C02_FIXTURES);
        fixture::run_fixtures_with::<Wdc65c02s>(fixture::MOS_FIXTURES);
    }
}
