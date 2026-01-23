/**
 * Maps expense names to relevant emojis based on keywords
 */
export function getExpenseEmoji(expenseName: string): string {
	const name = expenseName.toLowerCase();

	// Food & Drinks
	if (name.match(/restaurant|resto|dinner|lunch|breakfast|meal|food|eat/)) return '🍽️';
	if (name.match(/coffee|café|starbucks|tea/)) return '☕';
	if (name.match(/pizza/)) return '🍕';
	if (name.match(/burger/)) return '🍔';
	if (name.match(/sushi/)) return '🍣';
	if (name.match(/beer|bar|pub|bière|biere|drink|wine|alcohol/)) return '🍺';
	if (name.match(/grocery|groceries|supermarket|market|food shopping|courses/)) return '🛒';
	if (name.match(/ice cream|dessert/)) return '🍦';

	// Transportation
	if (name.match(/uber|taxi|cab|ride/)) return '🚕';
	if (name.match(/gas|fuel|essence|petrol/)) return '⛽';
	if (name.match(/train|railway/)) return '🚆';
	if (name.match(/plane|flight|airplane/)) return '✈️';
	if (name.match(/bus/)) return '🚌';
	if (name.match(/car|vehicle|auto/)) return '🚗';
	if (name.match(/bike|bicycle/)) return '🚲';
	if (name.match(/parking/)) return '🅿️';

	// Accommodation
	if (name.match(/hotel|airbnb|accommodation|lodging/)) return '🏨';
	if (name.match(/rent|loyer/)) return '🏠';

	// Entertainment
	if (name.match(/movie|cinema|film/)) return '🎬';
	if (name.match(/concert|music|festival/)) return '🎵';
	if (name.match(/game|gaming/)) return '🎮';
	if (name.match(/ski|skiing|snowboard/)) return '🎿';
	if (name.match(/sport|gym|fitness/)) return '⚽';
	if (name.match(/ticket|billet/)) return '🎟️';

	// Shopping
	if (name.match(/shop|shopping|clothes|clothing|fashion/)) return '🛍️';
	if (name.match(/phone|mobile|smartphone/)) return '📱';
	if (name.match(/computer|laptop/)) return '💻';
	if (name.match(/book|library/)) return '📚';

	// Services
	if (name.match(/internet|wifi/)) return '📡';
	if (name.match(/electricity|electric/)) return '⚡';
	if (name.match(/water|eau/)) return '💧';
	if (name.match(/insurance|assurance/)) return '🛡️';
	if (name.match(/medical|doctor|hospital|health|pharmacy/)) return '🏥';
	if (name.match(/haircut|salon|coiffeur/)) return '💇';
	if (name.match(/spa|massage|wellness/)) return '💆';

	// Gifts & Special
	if (name.match(/gift|cadeau|present/)) return '🎁';
	if (name.match(/birthday|anniversaire/)) return '🎂';
	if (name.match(/christmas|noël/)) return '🎄';

	// Default
	return '💵';
}
