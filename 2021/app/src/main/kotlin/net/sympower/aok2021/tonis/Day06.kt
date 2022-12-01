package net.sympower.aok2021.tonis

fun main() {
  println("1st: ${day06Part01("/Day06.in")}")
  println("2nd: ${day06Part02("/Day06.in")}")
}

fun day06Part01(fileIn: String): Long {
  return populationInDays(fileIn, 80)
}

fun day06Part02(fileIn: String): Long {
  return populationInDays(fileIn, 256)
}

private fun populationInDays(fileIn: String, totalDay: Int): Long {
  var population = Array(9) { 0L }

  readLineFromResource(fileIn) { line ->
    line.split(",").forEach { population[it.toInt()]++ }
  }

  repeat(totalDay) {
    val newPopulation = Array(9) { 0L }
    population.forEachIndexed { timer, populationSize ->
      when (timer) {
        0 -> {
          newPopulation[8] += populationSize
          newPopulation[6] += populationSize
        }
        else -> newPopulation[timer - 1] += populationSize
      }
    }
    population = newPopulation
  }

  return population.sum()
}