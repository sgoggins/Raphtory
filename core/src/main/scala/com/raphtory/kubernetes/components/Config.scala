package com.raphtory.kubernetes.components

import com.raphtory.Raphtory
import com.raphtory.kubernetes.utils
import com.raphtory.kubernetes.utils.KubernetesLogger
import com.typesafe.config

import java.util

/** Reads kubernetes configuration values from application.conf.
  */
class Config {
  val conf: config.Config          = Raphtory.getDefaultConfig(distributed = true)
  val raphtoryDeploymentId: String = conf.getString("raphtory.deploy.id")

  val raphtoryKubernetesNamespaceName: String =
    conf.getString("raphtory.deploy.kubernetes.namespace.name")

  val raphtoryKubernetesServiceAccountName: String =
    conf.getString("raphtory.deploy.kubernetes.serviceaccount.name")

  val raphtoryKubernetesDeployments: util.Set[String] =
    conf.getConfig("raphtory.deploy.kubernetes.deployments").root().keySet()
  val raphtoryKubernetesMasterUrl: String             = conf.getString("raphtory.deploy.kubernetes.master.url")

  val raphtoryKubernetesLogger: KubernetesLogger =
    utils.KubernetesLogger()

  val raphtoryKubernetesDockerRegistrySecretName: String =
    conf.getString("raphtory.deploy.kubernetes.secrets.registry.name")
}