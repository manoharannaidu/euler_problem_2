fn main() {
   let mut first: u32 = 1;
   let mut second: u32 = 2;
   let mut even_fib_sum: u32 = 2;
   loop {
      if (first + second) > 4000000 {
         break;
      }
      if (first + second)% 2 == 0 {
         even_fib_sum = even_fib_sum + (first + second);
      }
      let _next_fib = &first + &second;
      first = second;
      second = _next_fib;
   }
   println!("{even_fib_sum}");
}
