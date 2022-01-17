package playground

import spinal.core._
import spinal.lib._
import spinal.lib.com.uart._
import spinal.lib.graphic._

import playground.soc._
import playground.bsp.OrangeCrab

object RiscV {
  def main(args: Array[String]): Unit = OrangeCrab.generate(new RiscV)
}

case class RiscV() extends Component {
  val coreConfig = CoreConfig.withRamFile("src/main/resources/ram.hex")
  val io = new Bundle {

    val button = in(Bool)
    val aux = in(Bool)

    val reset = out(Bool)
    val pin0 = out(Bool)
    val pin1 = out(Bool)
    val pin5 = out(Bool)
    val pin6 = out(Bool)
    val pin9 = out(Bool)
    val pin10 = out(Bool)
    val pin11 = out(Bool)
    val pin12 = out(Bool)
    val pin13 = out(Bool)

    val uart = master(Uart())
    val led = out(Rgb(RgbConfig(1, 1, 1)))
  }

  io.reset := RegNext(io.button)

  val core = new Core(coreConfig)
  core.io.asyncReset <> ~io.aux
  core.io.uart <> io.uart
  core.io.led <> io.led

  val gpio = core.io.gpio
  gpio.read := 0
  gpio.write(0) <> io.pin0
  gpio.write(1) <> io.pin1
  gpio.write(5) <> io.pin5
  gpio.write(6) <> io.pin6
  gpio.write(9) <> io.pin9
  gpio.write(10) <> io.pin10
  gpio.write(11) <> io.pin11
  gpio.write(12) <> io.pin12
  gpio.write(13) <> io.pin13

}
