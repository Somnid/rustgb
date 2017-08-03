//TODO use num crate
pub trait AddSigned<RHS> {
	type Output;
	fn add_signed(self, rhs: RHS) -> Self::Output;
}

impl AddSigned<i8> for u16 {
	type Output = u16;
	fn add_signed(self, rhs: i8) -> u16 {
		if rhs < 0 {
			self - (rhs.abs() as u16)
		}else{
			self + (rhs as u16)
		}
	}
}

impl AddSigned<i32> for usize {
	type Output = usize;
	fn add_signed(self, rhs: i32) -> usize {
		if rhs < 0 {
			self - (rhs.abs() as usize)
		}else{
			self + (rhs as usize)
		}
	}
}
