package playground.bsp

import java.nio.file._
import spinal.core._
import spinal.lib._

object OrangeCrab {
  def generate[T <: Component](
      gen: => T,
      defaultClockDomainFrequency: HertzNumber = 48 MHz
  ) = {
    val targetDirectory = Paths.get("target/bitstream")
    if (!Files.exists(targetDirectory)) {
      Files.createDirectory(targetDirectory)
    }

    new SpinalConfig(
      defaultClockDomainFrequency = FixedFrequency(defaultClockDomainFrequency),
      defaultConfigForClockDomains = ClockDomainConfig(
        resetActiveLevel = LOW,
        resetKind = ASYNC
      ),
      targetDirectory = targetDirectory.toString()
    ).generateVerilog(gen)
  }
}
