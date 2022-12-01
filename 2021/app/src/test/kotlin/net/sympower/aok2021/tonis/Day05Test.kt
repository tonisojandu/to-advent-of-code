package net.sympower.aok2021.tonis

import org.assertj.core.api.Assertions.assertThat
import org.junit.jupiter.api.Test

class Day05Test {

  @Test
  fun test01() {
    val overlappingPoints = day05Part01("/Day05Test.in")

    assertThat(overlappingPoints).isEqualTo(5)
  }

  @Test
  fun test02() {
    val overlappingPoints = day05Part02("/Day05Test.in")

    assertThat(overlappingPoints).isEqualTo(12)
  }
}