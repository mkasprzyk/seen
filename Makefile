SHELL=/bin/sh
DOCS=./docs

docs-gen:
	dot -Tsvg ${DOCS}/eddystone-eid-explained.dot -o ${DOCS}/eddystone-eid-explained.svg

test:
	python2 -m unittest ephemeral_id.tests
