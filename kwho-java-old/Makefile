# invocation is like:
JAVAC=javac
JAR=jar

.PHONY: jar
jar: kwho.jar

.PHONY: java
java: kwho.class

kwho.jar: kwho.manifest kwho.class kwho.java
	$(JAR) cfm $@ $+

%.class: %.java
	$(JAVAC) $<
