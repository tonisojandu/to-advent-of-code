package net.sympower.aok2021.tonis

import kotlin.math.abs

fun main() {
  println("1st: ${day07Part01("/Day07.in")}")
  println("2nd: ${day07Part02("/Day07.in")}")
}

fun day07Part01(fileIn: String): Int {
  return calculateOptimalFuel(fileIn)
}

fun day07Part02(fileIn: String): Int {
  return calculateOptimalFuel(fileIn, progressiveFuelUsage = true)
}

fun calculateOptimalFuel(fileIn: String, progressiveFuelUsage: Boolean = false): Int {
  val crabPositions = readLineFromResource(fileIn) { line ->
    line.split(",")
      .map { it.toInt() }
  }.flatten()
    .sorted()
    .toIntArray()

  return findLocalMinimum(crabPositions.size / 2, crabPositions, progressiveFuelUsage)
}

fun findLocalMinimum(from: Int, crabPositions: IntArray, progressiveFuelUsage: Boolean): Int {
  val midFuelRequirement = calculateFuelRequirement(from, crabPositions, progressiveFuelUsage)
  if (from == 0 || from == crabPositions.size - 1) {
    return midFuelRequirement
  }
  if (midFuelRequirement > calculateFuelRequirement(from - 1, crabPositions, progressiveFuelUsage)) {
    return findLocalMinimum(from - 1, crabPositions, progressiveFuelUsage)
  } else if (midFuelRequirement > calculateFuelRequirement(from + 1, crabPositions, progressiveFuelUsage)) {
    return findLocalMinimum(from + 1, crabPositions, progressiveFuelUsage)
  }
  return midFuelRequirement
}

fun calculateFuelRequirement(position: Int, crabPositions: IntArray, progressiveFuelUsage: Boolean): Int {
  return crabPositions.sumOf { calculatePositionFuelUsage(it, position, progressiveFuelUsage) }
}

fun calculatePositionFuelUsage(position: Int, crabPosition: Int, progressiveFuelUsage: Boolean): Int =
  if (progressiveFuelUsage) {
    val diff = abs(crabPosition - position)
    (diff * (diff + 1)) / 2
  } else {
    abs(crabPosition - position)
  }