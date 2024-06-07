{
  "$schema": "http://json-schema.org/draft-07/schema",
  "title": "Test",
  "type": "object",
  "properties": {
    "name": {
      "title": "name",
      "description": "The name of the test.",
      "term": "schema:hello",
      "type": "string"
    },
    "number": {
      "title": "number",
      "term": "schema:one",
      "type": "number"
    },
    "test2": {
      "term": "schema:something",
      "type": "array",
      "items": {
        "$ref": "#/definitions/Test2"
      }
    },
    "ontology": {
      "title": "ontology",
      "$ref": "#/definitions/Ontology"
    }
  },
  "definitions": {
    "Ontology": {
      "title": "Ontology",
      "type": "string",
      "enum": [
        "https://www.evidenceontology.org/term/",
        "https://amigo.geneontology.org/amigo/term/",
        "http://semanticscience.org/resource/"
      ]
    },
    "Test2": {
      "title": "Test2",
      "type": "object",
      "properties": {
        "names": {
          "title": "names",
          "term": "schema:hello",
          "type": "array",
          "items": {
            "type": "string"
          }
        },
        "number": {
          "title": "number",
          "term": "schema:one",
          "type": "number",
          "minimum": 0.0
        }
      }
    }
  }
}