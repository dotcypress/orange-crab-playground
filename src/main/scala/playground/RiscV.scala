package playground

import spinal.core._
import spinal.lib._
import spinal.lib.com.uart._

import playground.soc._
import playground.bsp.OrangeCrab

object RiscV {
  def main(args: Array[String]): Unit = OrangeCrab.generate(new RiscV)
}

case class RiscV() extends Component {
  val coreConfig = CoreConfig.withRamFile("src/main/resources/ram.hex")
  val io = new Bundle {
    val button = in(Bool)
    val d0 = in(Bool)

    val reset = out(Bool)
    val uart = master(Uart())
    val led = out(new Bundle {
      val r = Bool
      val g = Bool
      val b = Bool
    })
  }

  io.reset := RegNext(io.button)

  val core = new Core(coreConfig)
  core.io.asyncReset <> ~io.d0
  core.io.uart <> io.uart

  val gpio = core.io.gpio

  gpio.write(0) <> io.led.r
  gpio.write(1) <> io.led.g
  gpio.write(2) <> io.led.b

  gpio.read := 0
}
