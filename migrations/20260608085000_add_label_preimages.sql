create table if not exists label_preimages (
  labelhash text primary key,
  label_name text not null
);

insert into label_preimages (labelhash, label_name)
select distinct on (labelhash) labelhash, label_name
from domains
where labelhash is not null
  and label_name is not null
order by labelhash, id
on conflict (labelhash) do update set label_name = excluded.label_name;

update domains as domain
set
  label_name = preimage.label_name,
  name = case
    when domain.parent_id is not null then preimage.label_name || '.' || (
      select parent.name from domains as parent where parent.id = domain.parent_id
    )
    else preimage.label_name
  end
from label_preimages as preimage
where domain.labelhash = preimage.labelhash
  and domain.label_name is null
  and (
    domain.parent_id is null
    or exists (
      select 1 from domains as parent
      where parent.id = domain.parent_id
        and parent.name is not null
    )
  );
