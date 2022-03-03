package com.raphtory.core.components.spout.executor

import com.raphtory.core.components.spout.SpoutExecutor
import com.raphtory.core.config.PulsarController
import com.typesafe.config.Config
import monix.execution.Scheduler
import org.apache.pulsar.client.api.Schema

import scala.io.Source

/** @DoNotDocument */
class ResourceSpoutExecutor(
    resource: String,
    conf: Config,
    pulsarController: PulsarController,
    scheduler: Scheduler
) extends SpoutExecutor[String](
                conf: Config,
                pulsarController: PulsarController,
                scheduler: Scheduler
        ) {

  private def readFile(fileDataPath: String): Unit = {
    // We assume that Pulsar standalone is running on the users machine before continuing
    // setup and create a producer
    val source   = Source.fromResource(fileDataPath)
    val producer = pulsarController.toBuildersProducer()
    for (line <- source.getLines())
      producer.sendAsync(kryo.serialise(line))
    source.close()
  }

  override def run(): Unit =
    readFile(resource)

  override def stop(): Unit =
    logger.debug(s"Stopping Resource Spout for resource '$resource'.")
}
