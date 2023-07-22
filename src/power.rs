use std::cmp::min;

use crate::error::create_error;
use crate::Result;

pub struct PowerBowls {
    bowls: [u32; 3],
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
            return Err(create_error("Not enough power on bowl 3"));
        }

        self.bowls[0] += amount;
        self.bowls[2] -= amount;

        Ok(())
    }

    pub fn burn(&mut self, amount: u32) -> Result<()> {
        if self.bowls[1] < amount * 2 {
            return Err(create_error("Not enough power on bowl 2"));
        }

        self.bowls[1] -= amount * 2;
        self.bowls[2] += amount;

        Ok(())
    }

    pub fn gain_limit(&self) -> u32 {
        2 * self.bowls[0] + self.bowls[1]
    }

    pub fn amount(&self, bowl: usize) -> u32 {
        self.bowls[bowl - 1]
    }
}

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
        let mut bowls = PowerBowls::new(4, 4, 4);

        bowls.burn(2)?;

        assert_eq!(bowls.amount(3), 6);
        assert_eq!(bowls.amount(2), 0);
        assert_eq!(bowls.amount(1), 4);

        Ok(())
    }

    #[test]
    fn try_burning_more_power_than_possible() -> Result<()> {
        let mut bowls = PowerBowls::new(3, 7, 2);

        assert!(bowls.burn(4).is_err());

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
}
