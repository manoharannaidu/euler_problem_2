fn main() {
   let mut first: u32 = 1;
   let mut second: u32 = 2;
   let mut even_fib_sum: u32 = 2;
   loop {
      let next_fib = first + second;
      if next_fib % 2 == 0 {
         even_fib_sum = even_fib_sum + next_fib;
      }
      first = second;
      second = next_fib;
      if next_fib > 4000000 {
         break;
      }
   }
   println!("{even_fib_sum}")
}
