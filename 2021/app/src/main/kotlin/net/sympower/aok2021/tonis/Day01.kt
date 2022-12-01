package net.sympower.aok2021.tonis

fun main() {
  val measurements = readLineFromResource("/Day01.in") { it.toInt() }

  val count = measurements.zipWithNext()
      .filter { (first, second) -> first < second }
      .size

  val windowCount = measurements.windowed(3)
      .map { (a, b, c) -> a + b + c }
      .zipWithNext()
      .filter { (first, second) -> first < second }
      .size

  println("""
    Counts: 
    * 1st: $count
    * 2nd: $windowCount
  """.trimIndent())
}