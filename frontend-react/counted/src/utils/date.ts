export const getPickerFormattedDate = (date: Date) => {
	return date.toISOString().slice(0, 10);
};
