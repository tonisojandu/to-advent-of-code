package net.sympower.aok2021.tonis

import org.assertj.core.api.Assertions
import org.junit.jupiter.api.Test

class Day06Test {

  @Test
  fun test01() {
    val totalFish = day06Part01("/Day06Test.in")

    Assertions.assertThat(totalFish).isEqualTo(5934)
  }

  @Test
  fun test02() {
    val totalFish = day06Part02("/Day06Test.in")

    Assertions.assertThat(totalFish).isEqualTo(26984457539)
  }
}