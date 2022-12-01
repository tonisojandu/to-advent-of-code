package net.sympower.aok2021.tonis

fun main() {

  println("1st: ${day02Part01("/Day02.in")})")
  println("2nd: ${day02Part02("/Day02.in")})")

}

fun day02Part01(fileIn: String): EndPosition {
  val movements = readMovements(fileIn)

  val totalMovements = movements.groupBy({ it.direction }) { it.amount }
      .map { it.key to it.value.sum() }
      .toMap()

  return EndPosition(totalMovements["down"]!! - totalMovements["up"]!!, totalMovements["forward"]!!)
}

fun day02Part02(fileIn: String): EndPosition {
  val movements = readMovements(fileIn)

  var aim = 0
  var depth = 0
  var progress = 0
  movements.forEach {
    when (it.direction) {
      "up" -> aim -= it.amount
      "down" -> aim += it.amount
      "forward" -> {
        depth += it.amount * aim
        progress += it.amount
      }
    }
  }

  return EndPosition(depth, progress)
}

private fun readMovements(fileIn: String): List<Movement> {
  return readLineFromResource(fileIn) {
    val parts = it.split(" ")
    Movement(parts[0], parts[1].toInt())
  }
}

data class Movement(val direction: String, val amount: Int)

data class EndPosition(val depth: Int, val progress: Int, val multiplied: Int = depth * progress)