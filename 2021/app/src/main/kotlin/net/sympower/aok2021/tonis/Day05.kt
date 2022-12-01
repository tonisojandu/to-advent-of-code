package net.sympower.aok2021.tonis

import java.lang.Integer.min
import kotlin.math.abs
import kotlin.math.max

fun main() {
  println("1st: ${day05Part01("/Day05.in")}")
  println("2nd: ${day05Part02("/Day05.in")}")
}

fun day05Part01(fileIn: String): Int {
  val lines = readLineFromResource(fileIn) { Line(it) }

  return lines.flatMap { it.getLinePoints() }.groupPaintCountIntersections()
}

fun day05Part02(fileIn: String): Int {
  val lines = readLineFromResource(fileIn) { Line(it) }

  return lines.flatMap { it.getLinePoints(horizontalOnly = false) }.groupPaintCountIntersections()
}

fun List<Pair<Int, Int>>.groupPaintCountIntersections(): Int {
  val result = this.groupBy { it }.map { it.key to it.value.size }.toMap()

  val maxX = this.maxOf { it.first }
  val maxY = this.maxOf { it.second }

  repeat(maxY + 1) { y ->
    List(maxX + 1) { x ->
      val count = result[x to y]
      if (count != null) {
        print(count)
      } else {
        print(".")
      }
    }
    println()
  }

  return result.filter { it.value > 1 }.count()
}

class Line(from: String) {

  private val from: Pair<Int, Int>
  private val to: Pair<Int, Int>

  init {
    val locationStrings = from.split(" -> ")

    this.from = parseCoordinate(locationStrings[0])
    this.to = parseCoordinate(locationStrings[1])
  }

  fun getLinePoints(horizontalOnly: Boolean = true): List<Pair<Int, Int>> = if (horizontalOnly) {
    listOf()
  } else {
    getDiagonalPoints()
  }.plus(getHorizontalPoints())

  private fun getDiagonalPoints(): List<Pair<Int, Int>> {
    val xDirection = if (to.first < from.first) -1 else 1
    val yDirection = if (to.second < from.second) -1 else 1
    val deltaX = abs(to.first - from.first)
    val deltaY = abs(to.second - from.second)
    return if (deltaX == deltaY && deltaX > 0) {
      List(deltaX + 1) { from.first + (xDirection * it) to from.second + (yDirection * it) }
    } else {
      listOf()
    }
  }

  private fun getHorizontalPoints() = if (from.first == to.first || from.second == to.second) {
    inclusiveGenerate(from.first, to.first).flatMap { x ->
      inclusiveGenerate(from.second, to.second).map { y -> x to y }
    }
  } else {
    listOf()
  }

  private fun inclusiveGenerate(from: Int, to: Int): List<Int> {
    val direction = if (to < from) -1 else 1
    return List(abs(to - from) + 1) { from + (direction * it) }
  }

  private fun getMinMax(first: Int, second: Int): Pair<Int, Int> {
    val from = min(first, second)
    val to = max(first, second)
    return from to to
  }

  private fun parseCoordinate(locationString: String): Pair<Int, Int> {
    val coordinateStrings = locationString.trim().split(",")
    return coordinateStrings[0].toInt() to coordinateStrings[1].toInt()
  }

}