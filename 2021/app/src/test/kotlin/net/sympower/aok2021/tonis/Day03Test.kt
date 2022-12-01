package net.sympower.aok2021.tonis

import org.assertj.core.api.Assertions.assertThat
import org.junit.jupiter.api.Test

class Day03Test {

  @Test
  fun test01() {
    val consumption = day03Part01("/Day03Test.in")

    assertThat(consumption).isEqualTo(198)
  }

  @Test
  fun test02() {
    val lifeSupportRating = day03Part02("/Day03Test.in")

    assertThat(lifeSupportRating).isEqualTo(230)
  }
}