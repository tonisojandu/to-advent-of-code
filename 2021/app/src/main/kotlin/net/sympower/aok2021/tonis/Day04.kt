package net.sympower.aok2021.tonis

fun main() {
  println("1st: ${day04Part01("/Day04.in")}")
  println("2nd: ${day04Part02("/Day04.in")}")
}

fun day04Part01(fileIn: String): Int {
  val (pickedNumbers, boards) = readNumbersAndBoards(fileIn)

  val (winningBoard, onNumber) = pickWinningBoard(boards, pickedNumbers)

  return onNumber * winningBoard.summarizeUnTicked()
}

fun day04Part02(fileIn: String): Int {
  val (pickedNumbers, boards) = readNumbersAndBoards(fileIn)

  val (winningBoard, onNumber) = pickLastBoard(boards, pickedNumbers)

  return onNumber * winningBoard.summarizeUnTicked()
}

fun readNumbersAndBoards(fileIn: String): Pair<List<Int>, List<Board>> {
  val lines = readLineFromResource(fileIn) { it }.toTypedArray()
  val pickedNumbers = lines[0].split(",").map { it.toInt() }

  val boards = Array(lines.size) { it }
    .groupBy(::lineBoardMembership) { lines[it] }
    .filter { it.key >= 0 }
    .map { Board(it.value) }

  return pickedNumbers to boards
}

fun pickWinningBoard(boards: List<Board>, pickedNumbers: List<Int>): Pair<Board, Int> {
  pickedNumbers.forEach { number ->
    boards.forEach { board ->
      if (board.isBingo(number)) {
        return board to number
      }
    }
  }
  throw IllegalStateException("Lottery sport wins!")
}

fun pickLastBoard(boards: List<Board>, pickedNumbers: List<Int>): Pair<Board, Int> {
  val boardsLeft = boards.toMutableSet()
  pickedNumbers.forEach { number ->
    val winners = boardsLeft.filter { board -> board.isBingo(number) }
    if (winners.size == boardsLeft.size) {
      return boardsLeft.first() to number
    }
    boardsLeft.removeAll(winners)
  }
  throw IllegalStateException("Lottery sport wins!")
}

fun lineBoardMembership(lineNumber: Int): Int {
  if (lineNumber <= 1) {
    return -1
  }
  if ((lineNumber - 2) % 6 == 5) {
    return -1
  }
  return (lineNumber - 2) / 6
}

class Board(lines: List<String>) {

  private val numbers: Array<Array<Int>>
  private val ticked: Array<Array<Boolean>>
  private val numberCoordinates: Map<Int, List<Pair<Int, Int>>>

  init {
    numbers = lines.map {
      it.replace(Regex("\\s+"), " ")
        .trim()
        .split(" ")
        .map { numIt -> numIt.toInt() }
        .toTypedArray()
    }.toTypedArray()

    ticked = numbers.map { it.map { false }.toTypedArray() }.toTypedArray()

    numberCoordinates = numbers.mapIndexed { rowIndex, row ->
      row.mapIndexed { columnIndex, number -> number to (rowIndex to columnIndex) }
    }
      .flatten()
      .groupBy({ it.first }) { it.second }

  }

  fun isBingo(number: Int): Boolean {
    numberCoordinates[number]?.let {
      return it.map { coordinates ->
        ticked[coordinates.first][coordinates.second] = true
        isRowBingo(coordinates.first) || isColumnBingo(coordinates.second)
      }.reduce { first, second -> first || second }
    }
    return false
  }

  fun summarizeUnTicked(): Int {
    return ticked.mapIndexed { rowIndex, row ->
      row.mapIndexed { columnIndex, isTicked ->
        if (!isTicked) {
          numbers[rowIndex][columnIndex]
        } else {
          0
        }
      }
    }.flatten()
      .sum()
  }

  private fun isRowBingo(rowIndex: Int): Boolean {
    return ticked[rowIndex]
      .reduce { first, second -> first && second }
  }

  private fun isColumnBingo(columnIndex: Int): Boolean {
    return ticked
      .map { row -> columnIndex < row.size && row[columnIndex] }
      .reduce { first, second -> first && second }
  }
}


