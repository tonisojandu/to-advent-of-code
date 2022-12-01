package net.sympower.aok2021.tonis

import org.assertj.core.api.Assertions
import org.junit.jupiter.api.Test

class Day04Test {

  @Test
  fun test01() {
    val score = day04Part01("/Day04Test.in")

    Assertions.assertThat(score).isEqualTo(4512)
  }

  @Test
  fun test02() {
    val score = day04Part02("/Day04Test.in")

    Assertions.assertThat(score).isEqualTo(1924)
  }
}