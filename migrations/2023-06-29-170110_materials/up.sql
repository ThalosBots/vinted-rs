-- Your SQL goes here
CREATE TABLE MATERIAL(
   id          INTEGER PRIMARY KEY,
   material_es VARCHAR(30) NOT NULL,
   material_fr VARCHAR(30) NOT NULL,
   material_en VARCHAR(30) NOT NULL
);

 INSERT INTO  MATERIAL(id,material_es,material_fr,material_en) VALUES
   (43,'Piel','Cuir','Leather'),
   (44,'Algodon','Coton','Cotton'),
   (45,'Poliester','Polyester','Polyester'),
   (46,'Lana','Laine','Wool'),
   (48,'Viscosa','Viscose','Viscose'),
   (49,'Seda','Soie','Silk'),
   (52,'Nailon','Nylon','Nylon'),
   (53,'Elastano','Elasthanne','Elastane'),
   (120,'Polar','Polaire','Fleece'),
   (121,'Merino','Merinos','Merino'),
   (122,'Alpaca','Alpaga','Alpaca'),
   (123,'Cachemira','Cachemire','Cashmere'),
   (146,'Lino','Lin','Linen'),
   (149,'Acrilico','Acrylique','Acrylic'),
   (152,'Angora','Mohair','Mohair'),
   (177,'Velveton','Peluche','Velour'),
   (178,'Neopreno','Neoprene','Neoprene'),
   (226,'Lentejuelas','Sequin','Sequin'),
   (298,'Ante','Daim','Suede'),
   (299,'Pana','Velous cotele','Corduroy'),
   (300,'Plastico','Plastique','Plastic'),
   (301,'Goma','Caoutchouc','Rubber'),
   (302,'Latex','Latex','Latex'),
   (303,'Vaquero','Denim','Denim'),
   (305,'Charol','Cuir verni','Patent leather'),
   (311,'Raso','Satin','Satin'),
   (440,'Bambu','Bambou','Bamboo'),
   (441,'Lona','Toile','Canvas'),
   (442,'Carton','Carton','Cardboard'),
   (443,'Ceramica','Ceramique','Ceramic'),
   (444,'Chifon','Mousseline','Chiffon'),
   (445,'Plumon','Duvet','Down'),
   (446,'Cuero sintetico','Fausse fourrure','Faux fur'),
   (447,'Piel sintetica','Cuir synthetique','Faux leather'),
   (448,'Fieltro','Feutre','Felt'),
   (449,'Gomaespuma','Mousse','Foam'),
   (451,'Franela','Flanelle','Flannel'),
   (452,'Cristal','Verre','Glass'),
   (453,'Oro','Or','Gold'),
   (454,'Yute','Jute','Jute'),
   (455,'Encaje','Dentelle','Lace'),
   (456,'Malla','Maille','Mesh'),
   (457,'Metal','Metal','Metal'),
   (458,'Papel','Papier','Paper'),
   (459,'Porcelana','Porcelaine','Porcelain'),
   (460,'Mimbre','Rotin','Rattan'),
   (461,'Plata','Argent','Silver'),
   (462,'Piedra','Pierre','Stone'),
   (463,'Paja','Paille','Straw'),
   (464,'Tul','Tulle','Tulle'),
   (465,'Tweed','Tweed','Tweed'),
   (466,'Terciopelo','Velours','Velvet'),
   (467,'Madera','Bois','Wood'),
   (470,'Silicona','Silicone','Silicone');
