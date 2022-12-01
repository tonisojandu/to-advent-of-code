package net.sympower.aok2021.tonis

import org.assertj.core.api.Assertions.assertThat
import org.junit.jupiter.api.Test

class Day02Test {

  @Test
  fun test01() {
    val endPosition = day02Part01("/Day02Test.in")

    assertThat(endPosition.multiplied).isEqualTo(150)
  }

  @Test
  fun test02() {
    val endPosition = day02Part02("/Day02Test.in")

    assertThat(endPosition.multiplied).isEqualTo(900)
  }
}