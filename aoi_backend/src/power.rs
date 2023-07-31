use std::cmp::min;

use crate::error::create_error;
use crate::common::{Books, Coins, Scholars, Tools};
use crate::Result;

#[derive(Clone, Copy)]
pub struct PowerBowls {
    bowls: [u32; 3],
}

pub struct PowerConversion {
    state_before: PowerBowls,
    state_after: PowerBowls,
    books_gained: Books,
    scholars_gained: Scholars,
    tools_gained: Tools,
    coins_gained: Coins,
}

impl PowerBowls {
    pub fn new(bowl1_amount: u32, bowl2_amount: u32, bowl3_amount: u32) -> Self {
        PowerBowls {
            bowls: [bowl1_amount, bowl2_amount, bowl3_amount],
        }
    }

    pub fn gain(&mut self, amount: u32) -> u32 {
        let mut total_gained = 0;

        let b1_to_b2_gain = min(self.bowls[0], amount);
        total_gained += b1_to_b2_gain;
        self.bowls[1] += b1_to_b2_gain;
        self.bowls[0] -= b1_to_b2_gain;

        let b2_to_b3_gain = min(self.bowls[1], amount - b1_to_b2_gain);
        total_gained += b2_to_b3_gain;
        self.bowls[2] += b2_to_b3_gain;
        self.bowls[1] -= b2_to_b3_gain;

        total_gained
    }

    pub fn spend(&mut self, amount: u32) -> Result<()> {
        if self.bowls[2] < amount {
            return Err(create_error("Not enough power in bowl 3"));
        }

        self.bowls[0] += amount;
        self.bowls[2] -= amount;

        Ok(())
    }

    pub fn gain_limit(&self) -> u32 {
        2 * self.bowls[0] + self.bowls[1]
    }

    pub fn amount(&self, bowl: usize) -> u32 {
        self.bowls[bowl - 1]
    }
}

impl PowerConversion {
    pub fn convert_to_coins(&mut self, amount: Coins) -> Result<()> {
        self.state_after.spend(amount.0)?;
        self.coins_gained.0 += amount.0;

        Ok(())
    }

    pub fn convert_to_tools(&mut self, amount: Tools) -> Result<()> {
        self.state_after.spend(3 * amount.0)?;
        self.tools_gained.0 += amount.0;

        Ok(())
    }

    pub fn convert_to_scholars(&mut self, amount: Scholars) -> Result<()> {
        self.state_after.spend(5 * amount.0)?;
        self.scholars_gained.0 += amount.0;

        Ok(())
    }

    pub fn convert_to_books(&mut self, amount: Books) -> Result<()> {
        self.state_after.spend(5 * amount.0)?;
        self.books_gained.0 += amount.0;

        Ok(())
    }

    pub fn burn_power(&mut self, amount: u32) -> Result<()> {
        if self.state_after.bowls[1] < amount * 2 {
            return Err(create_error("Not enough power in bowl 2"));
        }

        self.state_after.bowls[1] -= amount * 2;
        self.state_after.bowls[2] += amount;

        Ok(())
    }
}

pub fn start_conversion(bowls: PowerBowls) -> PowerConversion {
    PowerConversion {
        state_before: bowls,
        state_after: bowls.clone(),
        books_gained: Books(0),
        scholars_gained: Scholars(0),
        tools_gained: Tools(0),
        coins_gained: Coins(0),
    }
}

pub fn finish_conversion(conv: PowerConversion) -> (PowerBowls, Books, Scholars, Tools, Coins) {
    (conv.state_after, conv.books_gained, conv.scholars_gained, conv.tools_gained, conv.coins_gained)
}

pub fn abort_conversion(conv: PowerConversion) -> PowerBowls {
    conv.state_before
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gain_power_from_multiple_bowls() {
        let mut bowls = PowerBowls::new(3, 9, 0);

        bowls.gain(5);

        assert_eq!(bowls.amount(3), 2);
        assert_eq!(bowls.amount(2), 10);
        assert_eq!(bowls.amount(1), 0);
        assert_eq!(bowls.gain_limit(), 10);
    }

    #[test]
    #[should_panic]
    fn panic_when_getting_power_out_of_bounds() {
        let bowls = PowerBowls::new(0, 0, 0);

        bowls.amount(0);
    }

    #[test]
    fn gain_from_only_bowl2() {
        let mut bowls = PowerBowls::new(0, 7, 2);

        bowls.gain(3);

        assert_eq!(bowls.amount(3), 5);
        assert_eq!(bowls.amount(2), 4);
        assert_eq!(bowls.amount(1), 0);
        assert_eq!(bowls.gain_limit(), 4);
    }

    #[test]
    fn gain_from_only_bowl1() {
        let mut bowls = PowerBowls::new(9, 1, 2);

        bowls.gain(3);

        assert_eq!(bowls.amount(3), 2);
        assert_eq!(bowls.amount(2), 4);
        assert_eq!(bowls.amount(1), 6);
        assert_eq!(bowls.gain_limit(), 16);
    }

    #[test]
    fn gain_result_when_not_capped() {
        let mut bowls = PowerBowls::new(9, 1, 2);

        let amount_gained = bowls.gain(3);

        assert_eq!(amount_gained, 3);
    }

    #[test]
    fn gain_result_when_capped() {
        let mut bowls = PowerBowls::new(1, 0, 2);

        let amount_gained = bowls.gain(3);

        assert_eq!(amount_gained, 2);
    }

    #[test]
    fn burn_power() -> Result<()> {
        let bowls = PowerBowls::new(4, 4, 4);
        let mut conv = start_conversion(bowls);

        conv.burn_power(2)?;
        let (bowls, _, _, _, _) = finish_conversion(conv);

        assert_eq!(bowls.amount(3), 6);
        assert_eq!(bowls.amount(2), 0);
        assert_eq!(bowls.amount(1), 4);

        Ok(())
    }

    #[test]
    fn try_burning_more_power_than_possible() -> Result<()> {
        let bowls = PowerBowls::new(3, 7, 2);
        let mut conv = start_conversion(bowls);

        assert!(conv.burn_power(4).is_err());

        Ok(())
    }

    #[test]
    fn spend_power() -> Result<()> {
        let mut bowls = PowerBowls::new(4, 4, 4);

        bowls.spend(3)?;

        assert_eq!(bowls.amount(3), 1);
        assert_eq!(bowls.amount(2), 4);
        assert_eq!(bowls.amount(1), 7);

        Ok(())
    }

    #[test]
    fn spend_while_not_having_enough_power() -> Result<()> {
        let mut bowls = PowerBowls::new(4, 4, 4);

        assert!(bowls.spend(5).is_err());

        assert_eq!(bowls.amount(3), 4);
        assert_eq!(bowls.amount(2), 4);
        assert_eq!(bowls.amount(1), 4);

        Ok(())
    }

    #[test]
    fn conversion_of_resources() -> Result<()> {
        let bowls = PowerBowls::new(0, 5, 25);
        let mut conv = start_conversion(bowls);

        conv.convert_to_coins(Coins(1))?; // 1 power
        conv.convert_to_books(Books(2))?; // 10 power
        conv.convert_to_tools(Tools(2))?; // 6 power
        conv.convert_to_coins(Coins(2))?; // 2 power
        conv.convert_to_scholars(Scholars(1))?; // 5 power
        let (bowls, books, scholars, tools, coins) = finish_conversion(conv);

        assert_eq!(bowls.amount(3), 1);
        assert_eq!(bowls.amount(2), 5);
        assert_eq!(bowls.amount(1), 24);
        assert_eq!(books.0, 2);
        assert_eq!(scholars.0, 1);
        assert_eq!(tools.0, 2);
        assert_eq!(coins.0, 3);

        Ok(())
    }

    #[test]
    fn burn_and_convert_power() -> Result<()> {
        let bowls = PowerBowls::new(0, 5, 1);
        let mut conv = start_conversion(bowls);

        conv.burn_power(2)?;
        conv.convert_to_tools(Tools(1))?;
        let (bowls, books, scholars, tools, coins) = finish_conversion(conv);

        assert_eq!(bowls.amount(3), 0);
        assert_eq!(bowls.amount(2), 1);
        assert_eq!(bowls.amount(1), 3);
        assert_eq!(books.0, 0);
        assert_eq!(scholars.0, 0);
        assert_eq!(tools.0, 1);
        assert_eq!(coins.0, 0);

        Ok(())
    }
}
