/* practice10_itertools by rust(cargo)
 * 		written by Matsumoto Kazuki
 *
 *
 *
 */

extern crate itertools;

use itertools::*;


fn main()
{
	let x = vec!(0, 1, 2, 3, 4, 5, 7);

/* イテレーターをタプルで分解するメソッド(tuples(n))
 *  n個のタプルに分解する
 * ２がデフォルト，n個にしたいときには( tuples::<(_, _, …, _, _)>() )とする
 * "_"：アンダーバーには型を明示して型指定も可能
 * 余りの要素は切り捨てる
 */
// 2タプルver
	let tuple_vec = x.iter().tuples().map( | (prev, next) | prev + next).collect::<Vec<_>>();
	let tuple_sum :i32 = tuple_vec.iter().sum();

	println!("{:?} { }", tuple_vec, tuple_sum);
// 3タプルver
	let tuple_vec = x.iter().tuples::<(_, _, _)>().map( | (x1, x2, x3) | x1 + x2 + x3).collect::<Vec<_>>();
	let tuple_sum :i32 = tuple_vec.iter().sum();

	println!("{:?} { }", tuple_vec, tuple_sum);

/* イテレーターをタプルで切っていくメソッド(tuple_windows())
 * 現在のイテレーターからn個分のタプルを取り出す
 * ２がデフォルトで，n個にしたいときには( tuple_windows::<(_, _, …, _, _)>() )とする
 * "_"：アンダーバーには型を明示して型指定も可能
 * 切り出しの範囲が末端まで届いた時点で終了
 */
// 2タプルver
	let tuple_window_vec = x.iter().tuple_windows().map( | (current, next) | current + next).collect::<Vec<_>>();
	let tuple_window_sum :i32 = tuple_window_vec.iter().sum();

	println!("{:?} { }", tuple_window_vec, tuple_window_sum);
// 4タプルver
	let tuple_window_vec = x.iter().tuple_windows::<(_, _, _, _)>()
											.map( | (x1, x2, x3, x4) | x1 + x2 + x3 + x4)
											.collect::<Vec<_>>();
	let tuple_window_sum :i32 = tuple_window_vec.iter().sum();

    println!("{:?} { }", tuple_window_vec, tuple_window_sum);
}
