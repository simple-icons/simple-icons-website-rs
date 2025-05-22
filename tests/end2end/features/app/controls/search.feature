Feature: Search for icons
	Search for icons using the search control.

	Background:
		Given I see the app

	Scenario: Search for an icon given its name
		When I type "simple icons" in the search input
		Then I see the icon "Simple Icons" first

	Scenario: Search for an icon given its slug
		When I type "dotnet" in the search input
		Then I see the icon ".NET" first

	Scenario: Search for an icon given its AKA alias
		When I type "drawio" in the search input
		Then I see the icon "diagrams.net" first

	Scenario: Search for an icon given its duplicate alias
		When I type "gotowebinar" in the search input
		Then I see the icon "GoToMeeting" first

	Scenario: Search for an icon given its localized alias
		When I type "КиноПоиск" in the search input
		Then I see the icon "Kinopoisk" first
