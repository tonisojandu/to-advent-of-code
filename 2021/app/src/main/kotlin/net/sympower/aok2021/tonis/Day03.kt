package net.sympower.aok2021.tonis

fun main() {
  println("1st: ${day03Part01("/Day03.in")}")
  println("2nd: ${day03Part02("/Day03.in")}")
}

fun day03Part01(fileIn: String): Int {
  val numbers = readLineFromResource(fileIn) { it }
  val threshold = numbers.size / 2

  val overThreshold = numbers
    .map { it.toCharArray() }
    .flatMap { chars -> Array(chars.size) { it }.filter { chars[chars.size - it - 1] == '1' } }
    .groupBy { it }
    .map { it.key to it.value.size }
    .filter { it.second > threshold } // no explanation if there are equal amounts
    .map { it.first }
    .toSet()

  val maxShift = overThreshold.maxOf { it }

  val gammaRate = overThreshold.shiftAndReduce()
  val epsilonRate = Array(maxShift) { it }.filter { !overThreshold.contains(it) }.shiftAndReduce()

  return gammaRate * epsilonRate
}

fun day03Part02(fileIn: String): Int {
  val numbers = readLineFromResource(fileIn) { it.toCharArray() }

  val maxLength = numbers.maxOf { it.size }

  val largest = Array(maxLength) { it }
    .fold(numbers) { lastLargest, onIndex -> keepLargest(lastLargest, onIndex) }
    .first()
    .binaryToInt()
  val smallest = Array(maxLength) { it }
    .fold(numbers) { lastLargest, onIndex -> keepSmallest(lastLargest, onIndex) }
    .first()
    .binaryToInt()

  return largest * smallest
}

fun keepSmallest(from: List<CharArray>, onIndex: Int): List<CharArray> {
  return preferGroup(from, onIndex, '0') { it.value.size }
}

fun keepLargest(from: List<CharArray>, onIndex: Int): List<CharArray> {
  return preferGroup(from, onIndex, '1') { -it.value.size }
}

fun preferGroup(
  from: List<CharArray>,
  onIndex: Int,
  finallyPreferring: Char,
  preferring: (Map.Entry<Char, List<CharArray>>) -> Int
): List<CharArray> {
  if (from.size == 1) {
    return from
  }
  val groupsOrderedBySize = from.groupBy { it[onIndex] }
    .entries
    .sortedBy(preferring)
    .map { it.value }
  if (groupsOrderedBySize[0].size == groupsOrderedBySize[1].size) {
    return if (groupsOrderedBySize[0][0][onIndex] == finallyPreferring) {
      groupsOrderedBySize[0]
    } else {
      groupsOrderedBySize[1]
    }
  }
  return groupsOrderedBySize.first()
}

fun CharArray.binaryToInt(): Int {
  return Array(this.size) { it }
    .filter { this[this.size - it - 1] == '1' }
    .shiftAndReduce()
}

fun Iterable<Int>.shiftAndReduce(): Int {
  return this.map { 1 shl it }.reduce { a, b -> a or b }
}