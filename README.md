# cut
cmd like unix cut get par of a fils/stdin



Options	            Description
-f (- champs)	    Spécifiez les champs que vous souhaitez extraire.
-c (- caractères)	Spécifiez les caractères que vous souhaitez extraire.
-b (- octets)	    Fournissez les octets que vous souhaitez extraire.
-d (-Délimiteur)	Ici, vous spécifiez le délimiteur que vous souhaitez utiliser avec la commande CUT.
Par défaut, l'onglet est considéré comme un délimiteur.
--complement	    Il est utilisé pour obtenir des colonnes non spécifiées par -f, -c ou -b Options.
--output-delimiter	Par défaut, la commande CUT utilise le délimiteur d'entrée en tant que délimiteur de sortie,
mais vous pouvez modifier ce comportement en utilisant cette option.

https://www.malekal.com/la-commande-cut-linux-utilisation-et-exemples/
TODO : manage errors
manager all syntaxes ex -c 1-3 -> char 1 to 3    -c 3-   -> char after 3     -c 1,3,5,7 -> chars 1 3 5 and 7
...
