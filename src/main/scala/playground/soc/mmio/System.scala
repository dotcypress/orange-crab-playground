package playground.soc.mmio

import spinal.core._
import spinal.lib._
import spinal.lib.bus.amba3.apb._
import spinal.lib.bus.misc.BusSlaveFactory
import spinal.lib.graphic.Rgb

case class Apb3SystemCtrl() extends Component {
  val io = new Bundle {
    val apb = slave(Apb3(addressWidth = 8, dataWidth = 32))
    val panic = out(Bool)
  }

  val sysClockFreq = S(clockDomain.frequency.getValue.toInt)
  val sysClockPeriod = S(1000000000 / clockDomain.frequency.getValue.toInt)

  val busCtrl = Apb3SlaveFactory(io.apb)
  busCtrl.read(sysClockFreq, 0x00)
  busCtrl.read(sysClockPeriod, 0x04)
}
