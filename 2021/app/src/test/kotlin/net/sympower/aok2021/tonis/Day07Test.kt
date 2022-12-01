package net.sympower.aok2021.tonis

import org.assertj.core.api.Assertions.assertThat
import org.junit.jupiter.api.Test

class Day07Test {

  @Test
  fun test01() {
    val amountOfFuel = day07Part01("/Day07Test.in")

    assertThat(amountOfFuel).isEqualTo(37)
  }

  @Test
  fun test02() {
    val amountOfFuel = day07Part02("/Day07Test.in")

    assertThat(amountOfFuel).isEqualTo(168)
  }
}