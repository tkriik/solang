(use person)
(use address)
(use name)

(def persons
  [(person/->person (name/->name "John" "Doe")
                    (address/->address "Foostreet" "6A" "512"))
   (person/->person (name/->name "Marilyn" "Monroe")
                    (address/->address "Barstreet" "1B" "256"))])

(def main
  (trace "persons" persons))
