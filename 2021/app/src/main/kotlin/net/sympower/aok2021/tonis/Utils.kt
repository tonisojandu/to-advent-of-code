package net.sympower.aok2021.tonis

fun <T> readLineFromResource(resourceName: String, parse: (String) -> T): List<T> {
  val resource = Utils::class.java.getResource(resourceName)
      ?: throw IllegalArgumentException("No resource with name $resourceName")

  return resource.readText()
      .lines()
      .map(parse)
}

class Utils