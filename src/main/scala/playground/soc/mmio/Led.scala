package playground.soc.mmio

import spinal.core._
import spinal.lib._
import spinal.lib.bus.amba3.apb._
import spinal.lib.bus.misc.BusSlaveFactory
import spinal.lib.graphic._

case class PwmLedCtrl() extends Component {
  val io = new Bundle {
    val enable = in(Bool)
    val colors = slave(Flow(Rgb(4, 4, 4)))
    val led = out(Rgb(RgbConfig(1, 1, 1)))
  }

  val gamma = Vec[UInt](
    0, 3, 7, 10, 18, 28, 41, 56, 73, 92, 113, 137, 163, 192, 222, 255
  )

  val disable = ~io.enable.asUInt
  val counter = Counter(8 bit, io.enable)
  val color = io.colors.toReg()

  io.led.r := disable | RegNext(counter >= gamma(color.r)).asUInt
  io.led.g := disable | RegNext(counter >= gamma(color.g)).asUInt
  io.led.b := disable | RegNext(counter >= gamma(color.b)).asUInt
}

case class Apb3LedCtrl() extends Component {
  val io = new Bundle {
    val apb = slave(Apb3(addressWidth = 8, dataWidth = 32))
    val led = out(Rgb(RgbConfig(1, 1, 1)))
  }

  val rgb = Rgb(4, 4, 4)
  val busCtrl = Apb3SlaveFactory(io.apb)

  val leds = new PwmLedCtrl()
  leds.io.led <> io.led

  val enable = busCtrl.createReadAndWrite(Bool, 0x00, 0)
  leds.io.enable := enable
  leds.io.colors.valid := enable

  leds.io.colors.payload := busCtrl
    .createReadAndWrite(Bits(12 bits), 0x00, 16)
    .as(rgb)

}
