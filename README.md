# Protocolo MQTT 

Proyecto 2C-2021 de la materia Taller de Programación, catedra Deymonnaz.

MQTT es un protocolo de mensajería basado en el patrón de comunicación publisher-suscriber y la
arquitectura cliente-servidor. Dentro de sus principales características se puede destacar que es un
protocolo liviano, simple y sencillo de implementar. Es un protocolo de capa de aplicación binario
construido sobre TCP/IP. Está basado en el patrón de comunicación publisher-subscriber. Esencialmente, es un patrón
en el que existen clientes que quieren comunicar mensajes a través de tópicos, los cuáles son
entregados a otros clientes que se encuentran suscritos a estos tópicos. Los clientes (publicadores y
suscriptores) no se conocen entre sí, sino que envían los mensajes pertinentes con su respectivo
tópico únicamente al servidor, cuya principal tarea es entregar los mensajes a los clientes que
corresponda.
