import postgresql
from postgresql.driver.dbapi20 import Cursor as CompatCursor

from semantix.caos.backends.meta import MetaError


class DatabaseTable(object):
    def __init__(self, connection):
        self.connection = connection
        self.cursor = CompatCursor(connection)

    def create(self):
        if self.create.__doc__ is None:
            raise Exception('missing table definition in docstring')

        try:
            self.runquery(self.create.__doc__)
        except postgresql.exceptions.DuplicateTableError:
            pass

    def insert(self, *dicts, **kwargs):
        data = {}
        for d in dicts + (kwargs,):
            data.update(d)

        if self.insert.__doc__ is None:
            raise Exception('missing insert statement in docstring')

        result = self.runquery(self.insert.__doc__, data)

        return result

    def runquery(self, query, params=None):
        query, pxf, nparams = self.cursor._convert_query(query)
        ps = self.connection.prepare(query)
        if params:
            return ps(*pxf(params))
        else:
            return ps()

class ConceptTable(DatabaseTable):
    def create(self):
        """
            CREATE TABLE "caos"."concept"(
                id serial NOT NULL,
                name text NOT NULL,

                PRIMARY KEY (id)
            )
        """
        super(ConceptTable, self).create()

    def insert(self, *dicts, **kwargs):
        """
            INSERT INTO "caos"."concept"(name) VALUES (%(name)s) RETURNING id
        """
        super(ConceptTable, self).insert(*dicts, **kwargs)

class ConceptMapTable(DatabaseTable):
    def create(self):
        """
            CREATE TABLE "caos"."concept_map"(
                id serial NOT NULL,
                source_id integer NOT NULL,
                target_id integer NOT NULL,
                link_type varchar(255) NOT NULL,
                mapping char(2) NOT NULL,
                required boolean NOT NULL DEFAULT FALSE,

                PRIMARY KEY (id),
                FOREIGN KEY (source_id) REFERENCES "caos"."concept"(id) ON DELETE CASCADE,
                FOREIGN KEY (target_id) REFERENCES "caos"."concept"(id) ON DELETE CASCADE
            )
        """
        super(ConceptMapTable, self).create()

    def insert(self, *dicts, **kwargs):
        """
            INSERT INTO "caos"."concept_map"(source_id, target_id, link_type, mapping, required)
                VALUES (
                            (SELECT id FROM caos.concept WHERE name = %(source)s),
                            (SELECT id FROM caos.concept WHERE name = %(target)s),
                            %(link_type)s,
                            %(mapping)s,
                            %(required)s
                ) RETURNING id
        """
        super(ConceptMapTable, self).insert(*dicts, **kwargs)


class EntityTable(DatabaseTable):
    def create(self):
        """
            CREATE TABLE "caos"."entity"(
                id serial NOT NULL,
                concept_id integer NOT NULL,

                PRIMARY KEY (id),
                FOREIGN KEY (concept_id) REFERENCES "caos"."concept"(id)
            )
        """
        super(EntityTable, self).create()

    def insert(self, *dicts, **kwargs):
        raise MetaError('direct inserts into entity table are not allowed')


class EntityMapTable(DatabaseTable):
    def create(self):
        """
            CREATE TABLE "caos"."entity_map"(
                source_id integer NOT NULL,
                target_id integer NOT NULL,
                link_type_id integer NOT NULL,
                weight integer NOT NULL,

                PRIMARY KEY (source_id, target_id, link_type_id)
            )
        """
        super(EntityMapTable, self).create()


class PathCacheTable(DatabaseTable):
    def create(self):
        """
            CREATE TABLE caos.path_cache (
                id                  serial NOT NULL,

                entity_id           integer NOT NULL,
                parent_entity_id    integer,

                name_attribute      varchar(255),
                concept_name        varchar(255) NOT NULL,

                weight              integer,

                PRIMARY KEY (id),
                UNIQUE(entity_id, parent_entity_id),

                FOREIGN KEY (entity_id) REFERENCES caos.entity(id)
                    ON UPDATE CASCADE ON DELETE CASCADE,

                FOREIGN KEY (parent_entity_id) REFERENCES caos.entity(id)
                    ON UPDATE CASCADE ON DELETE CASCADE
            )
        """
        super(PathCacheTable, self).create()

    def insert(self, *dicts, **kwargs):
        """
            INSERT INTO
                caos.path_cache
                    (entity_id, parent_entity_id, name_attribute, concept_name, weight)

                VALUES(%(entity_id)s, %(parent_entity_id)s,
                       %(name_attribute)s, %(concept_name)s, %(weight)s)
            RETURNING entity_id
        """
        return super(PathCacheTable, self).insert(*dicts, **kwargs)
