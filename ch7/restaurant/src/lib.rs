mod front_of_house {
  pub mod hosting {
    pub fn add_to_waitlist() {}
    fn seat_at_table() {}
  }
}
pub fn eat_at_restaurant() {
    // abs
    crate::front_of_house::hosting::add_to_waitlist();
    //rel
    front_of_house::hosting::add_to_waitlist();
  }
//  mod serving  {
  //      fn take_order() {}

    //    fn serve_order() {}

      //  fn take_payment() {}
//    }
//}//

