-- Notes from six customer interviews, one row per tagged note.
CREATE TABLE notes (interview_id INTEGER, tag TEXT, quote TEXT);
INSERT INTO notes VALUES
 (1,'slow-checkout','Paying took forever'),
 (1,'slow-checkout','Again, the checkout page hung'),
 (1,'no-invoice','I could not get an invoice'),
 (2,'slow-checkout','Checkout is slow on my phone'),
 (2,'confusing-pricing','Which tier do I need?'),
 (3,'confusing-pricing','Pricing page is a maze'),
 (3,'slow-checkout','Spinner for ten seconds'),
 (3,'no-invoice','Finance needs invoices'),
 (4,'no-invoice','No PDF invoice'),
 (4,'dark-mode','Would like dark mode'),
 (5,'confusing-pricing','Tiers overlap'),
 (5,'dark-mode','Eyes hurt at night'),
 (6,'no-invoice','Still no invoice export'),
 (6,'confusing-pricing','Unclear what is included');
