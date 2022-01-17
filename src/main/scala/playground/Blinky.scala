package playground

import spinal.core._
import spinal.lib._
import spinal.lib.graphic._
import playground.bsp.OrangeCrab

object Blinky {
  def main(args: Array[String]): Unit = OrangeCrab.generate(new Blinky)
}

case class Blinky() extends Component {
  val io = new Bundle {
    val button = in(Bool)

    val led = out(Rgb(RgbConfig(1, 1, 1)))
    val reset = out(Bool)
  }

  io.reset := RegNext(io.button)

  new SlowArea(1 Hz) {
    val colors = Reg(Bits(3 bits)) init (6)
    io.led.r := U(colors(0))
    io.led.g := U(colors(1))
    io.led.b := U(colors(2))
    colors := colors.rotateLeft(1)
  }
}
